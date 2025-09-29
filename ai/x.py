# rail_ledger_three_tabs.py
# Streamlit app with three sections: Vendors, Users, Batches
# Uses local CSVs: data/vendors.csv, data/users.csv, data/batches.csv
# Run: streamlit run rail_ledger_three_tabs.py

import os
from datetime import date
from typing import Tuple, List

import numpy as np
import pandas as pd
import plotly.express as px
from sklearn.cluster import DBSCAN
import streamlit as st

# -------------------------- Page Config --------------------------
st.set_page_config(page_title="Rail Ledger – Vendor/User/Batch Insights", layout="wide")

DATA_DIR = "data"
VENDORS_PATH = os.path.join(DATA_DIR, "vendors.csv")
USERS_PATH   = os.path.join(DATA_DIR, "users.csv")
BATCHES_PATH = os.path.join(DATA_DIR, "batches.csv")

# -------------------------- Helpers --------------------------
@st.cache_data
def load_csv_local(path: str) -> pd.DataFrame:
    if not os.path.exists(path):
        return pd.DataFrame()
    return pd.read_csv(path)

def coerce_date_cols(df: pd.DataFrame, cols: List[str]) -> pd.DataFrame:
    for c in cols:
        if c in df.columns:
            df[c] = pd.to_datetime(df[c], errors="coerce").dt.date
    return df

def haversine_km(lat1, lon1, lat2, lon2):
    R = 6371.0
    lat1, lon1, lat2, lon2 = map(np.radians, [lat1, lon1, lat2, lon2])
    dlat = lat2 - lat1
    dlon = lon2 - lon1
    a = np.sin(dlat/2.0)**2 + np.cos(lat1) * np.cos(lat2) * np.sin(dlon/2.0)**2
    return R * 2 * np.arcsin(np.sqrt(a))

def nearest_k(point_lat, point_lon, cand_df, lat_col, lon_col, k=3):
    if cand_df.empty:
        return cand_df
    d = cand_df.copy()
    d["distance_km"] = haversine_km(point_lat, point_lon, d[lat_col].values, d[lon_col].values)
    return d.sort_values("distance_km").head(k)

def cluster_env_stress(b: pd.DataFrame, eps_km: float, min_samples: int) -> Tuple[pd.Series, pd.DataFrame]:
    # Returns environment_stress series and cluster summary (centroids & sizes)
    if not {"fitment_lat", "fitment_lon"}.issubset(b.columns):
        return pd.Series(0.0, index=b.index), pd.DataFrame()

    coords = b[["fitment_lat", "fitment_lon"]].dropna()
    env_stress = pd.Series(0.0, index=b.index)
    if len(coords) < min_samples:
        return env_stress, pd.DataFrame()

    eps_deg = eps_km / 111.0  # rough conversion
    db = DBSCAN(eps=eps_deg, min_samples=min_samples, metric="euclidean")
    labels = db.fit_predict(b[["fitment_lat", "fitment_lon"]].fillna(0.0))
    b["cluster_label"] = labels

    # Compute riskiness per cluster (fraction risky)
    cluster_info = []
    for cl in np.unique(labels):
        mask = (labels == cl)
        if cl == -1:
            env = 0.0
        else:
            risky = ((b.loc[mask, "qc_penalty"] > 0) | (b.loc[mask, "age_ratio"] > 1)).mean()
            env = min(1.0, risky * (mask.sum() / max(1, len(b))) * 10.0)
        env_stress.loc[mask] = env

        if cl != -1:
            sub = b.loc[mask, ["fitment_lat", "fitment_lon", "risk_score"]]
            if not sub.empty:
                cluster_info.append({
                    "cluster": int(cl),
                    "count": int(sub.shape[0]),
                    "centroid_lat": float(sub["fitment_lat"].mean()),
                    "centroid_lon": float(sub["fitment_lon"].mean()),
                    "avg_risk": float(sub["risk_score"].mean())
                })

    cluster_df = pd.DataFrame(cluster_info).sort_values(["count", "avg_risk"], ascending=[False, False])
    return env_stress, cluster_df

# -------------------------- Sidebar Controls --------------------------
st.sidebar.title("⚙️ Controls")
sla_days = st.sidebar.number_input("Inspection SLA (days)", min_value=7, max_value=180, value=45, step=1)
replacement_risk_threshold = st.sidebar.slider("Replacement Risk Threshold", 0.0, 1.5, 0.7, 0.05)
cluster_eps_km = st.sidebar.slider("Hotspot radius (km)", 5, 200, 30, 5)
cluster_min_samples = st.sidebar.slider("Hotspot min samples", 2, 15, 3, 1)
qr_anomaly_distance_km = st.sidebar.slider("Duplicate-QR anomaly distance (km)", 10, 2000, 250, 10)

# -------------------------- Load Data --------------------------
vendors = load_csv_local(VENDORS_PATH)
users   = load_csv_local(USERS_PATH)
batches = load_csv_local(BATCHES_PATH)

if vendors.empty or users.empty or batches.empty:
    st.error(
        "Could not find required CSVs. Make sure these files exist:\n"
        f"- {VENDORS_PATH}\n- {USERS_PATH}\n- {BATCHES_PATH}"
    )
    st.stop()

vendors = coerce_date_cols(vendors, ["audit_date"])
batches = coerce_date_cols(batches, ["date_of_production", "expiry_date", "last_inspection_date", "fitment_date"])

today = date.today()

# -------------------------- Derivations (shared) --------------------------
# Vendor QC from batches
vendor_qc = (
    batches.assign(pass_flag=lambda d: d["qc_status"].astype(str).str.lower().eq("pass"))
           .groupby("vendor_id", as_index=False)
           .agg(total_batches=("batch_id","count"),
                pass_rate=("pass_flag","mean"))
)
vendors = vendors.merge(vendor_qc, on="vendor_id", how="left")
vendors["pass_rate"] = vendors["pass_rate"].fillna(0.7)
vendors["audit_age_days"] = vendors["audit_date"].apply(lambda d: (today - d).days if pd.notna(d) else np.nan)

# Batch metrics
b = batches.copy()
if "suggested_lifespan_months" not in b.columns:
    if {"expiry_date","date_of_production"}.issubset(b.columns):
        b["suggested_lifespan_months"] = ((pd.to_datetime(b["expiry_date"]) - pd.to_datetime(b["date_of_production"])).dt.days / 30).round().clip(lower=1).fillna(12).astype(int)
    else:
        b["suggested_lifespan_months"] = 12

b["days_since_production"] = (pd.to_datetime(today) - pd.to_datetime(b["date_of_production"])).dt.days
b["days_to_expiry"] = (pd.to_datetime(b["expiry_date"]) - pd.to_datetime(today)).dt.days
b["days_since_last_inspection"] = (pd.to_datetime(today) - pd.to_datetime(b["last_inspection_date"])).dt.days

qc_map = {"pass": 0.0, "pending inspection": 0.5, "pending": 0.5, "fail": 1.0}
b["qc_status_norm"] = b["qc_status"].astype(str).str.lower().str.strip()
b["qc_penalty"] = b["qc_status_norm"].map(qc_map).fillna(0.0)

b = b.merge(vendors[["vendor_id","pass_rate"]], on="vendor_id", how="left").rename(columns={"pass_rate":"vendor_pass_rate"})
b["vendor_pass_rate"] = b["vendor_pass_rate"].fillna(0.7)
b["vendor_risk"] = 1 - b["vendor_pass_rate"]

b["age_ratio"] = b["days_since_production"] / (b["suggested_lifespan_months"] * 30)
b["age_ratio"] = b["age_ratio"].clip(lower=0, upper=1.5)

b["insp_overdue"] = (b["days_since_last_inspection"] - sla_days) / max(1, sla_days)
b.loc[b["days_since_last_inspection"].isna(), "insp_overdue"] = 1.0
b["insp_overdue"] = b["insp_overdue"].clip(lower=0)

# Environment stress via clusters (computed below after we have b fields)
# Risk score
w_age, w_insp, w_qc, w_vendor, w_env = 0.35, 0.20, 0.20, 0.15, 0.10
b["environment_stress"] = 0.0  # placeholder, filled after clustering
b["risk_score"] = (
    w_age * b["age_ratio"].fillna(0) +
    w_insp * b["insp_overdue"].fillna(0) +
    w_qc * b["qc_penalty"].fillna(0) +
    w_vendor * b["vendor_risk"].fillna(0) +
    w_env * b["environment_stress"].fillna(0)
)

def risk_tier(x):
    if x > 0.7: return "High"
    if x > 0.4: return "Medium"
    return "Low"
b["risk_tier"] = b["risk_score"].apply(risk_tier)

# -------------------------- Tabs --------------------------
tab_vendor, tab_user, tab_batch = st.tabs(["🏷️ Vendors", "👤 Users", "📦 Batches"])

# ========================== VENDORS TAB ==========================
with tab_vendor:
    st.header("Vendor Insights – Governance & Quality")

    # KPIs
    total_vendors = vendors["vendor_id"].nunique()
    overdue_audits = vendors["audit_age_days"].gt(365).sum()
    vendor_risk_sum = b.groupby("vendor_id")["risk_score"].sum().reset_index(name="risk_sum")
    vendors_scored = vendors.merge(vendor_risk_sum, on="vendor_id", how="left")
    top_vendor = vendors_scored.sort_values("pass_rate", ascending=False).head(1)

    c1, c2, c3 = st.columns(3)
    c1.metric("Total Vendors", total_vendors)
    c2.metric("Vendors with Audit > 365 days", int(overdue_audits))
    if not top_vendor.empty:
        c3.metric("Top Vendor by QC Pass Rate", f"{top_vendor.iloc[0]['vendor_id']} ({top_vendor.iloc[0]['pass_rate']:.0%})")
    else:
        c3.metric("Top Vendor by QC Pass Rate", "—")

    # QC Pass rates bar
    fig_vendor = px.bar(
        vendors.sort_values("pass_rate", ascending=False),
        x="vendor_id", y="pass_rate",
        title="Vendor QC Pass Rate",
        hover_data=["city", "state", "audit_age_days", "no_of_batches"]
    )
    fig_vendor.update_yaxes(tickformat=".0%")
    st.plotly_chart(fig_vendor, use_container_width=True)

    # Scatter pass rate vs audit age
    vendors_scored["pass_rate_pct"] = 100 * vendors_scored["pass_rate"]
    fig_scatter = px.scatter(
        vendors_scored, x="audit_age_days", y="pass_rate_pct",
        size=vendors_scored["total_batches"].fillna(0)+1,
        color="state", hover_name="vendor_id",
        title="Pass Rate vs Audit Age (size ~ total batches)"
    )
    st.plotly_chart(fig_scatter, use_container_width=True)

    # Vendor map (if lat/lon present)
    if {"vendor_lat","vendor_lon"}.issubset(vendors.columns):
        vmap_df = vendors_scored.dropna(subset=["vendor_lat","vendor_lon"]).copy()
        vmap_df["bubble"] = np.clip((vmap_df["total_batches"].fillna(1) * 2), 6, 30)
        fig_vmap = px.scatter_mapbox(
            vmap_df, lat="vendor_lat", lon="vendor_lon",
            color="pass_rate", size="bubble",
            hover_name="vendor_id", hover_data=["city","state","pass_rate","total_batches"],
            zoom=4, height=500, title="Vendors by Location (size ~ total batches)"
        )
        fig_vmap.update_layout(mapbox_style="open-street-map")
        st.plotly_chart(fig_vmap, use_container_width=True)
    else:
        st.info("Vendor lat/lon not found for map.")

    # Preferred list & Watchlist
    preferred = vendors_scored.sort_values(["pass_rate","audit_age_days"], ascending=[False, True]).head(5)
    watchlist = vendors_scored[(vendors_scored["pass_rate"] < 0.70) | (vendors_scored["audit_age_days"] > 365)].copy()
    st.subheader("Recommended Actions")
    c1, c2 = st.columns(2)
    with c1:
        st.markdown("**Preferred Vendors (Top 5)**")
        st.dataframe(preferred[["vendor_id","city","state","pass_rate","audit_age_days","total_batches"]])
    with c2:
        st.markdown("**Watchlist (Low pass rate or audit overdue)**")
        st.dataframe(watchlist[["vendor_id","city","state","pass_rate","audit_age_days","total_batches"]])

# ========================== USERS TAB ==========================
with tab_user:
    st.header("User Insights – Coverage & Dispatch Readiness")

    # Role composition
    if "user_role" in users.columns:
        role_counts = users["user_role"].value_counts().reset_index()
        role_counts.columns = ["role","count"]
        fig_roles = px.pie(role_counts, names="role", values="count", title="User Roles")
        st.plotly_chart(fig_roles, use_container_width=True)
    else:
        st.info("No 'user_role' column in users.csv")

    # Coverage map vs hotspots
    # First compute environment stress clusters on batches and fill into risk score
    env_stress, clusters = cluster_env_stress(b, cluster_eps_km, cluster_min_samples)
    b["environment_stress"] = env_stress
    # Update risk with environment component
    b["risk_score"] = (
        w_age * b["age_ratio"].fillna(0) +
        w_insp * b["insp_overdue"].fillna(0) +
        w_qc * b["qc_penalty"].fillna(0) +
        w_vendor * b["vendor_risk"].fillna(0) +
        w_env * b["environment_stress"].fillna(0)
    )
    b["risk_tier"] = b["risk_score"].apply(risk_tier)

    st.subheader("Coverage Map: Users vs At-Risk Fitments")
    if {"base_lat","base_lon"}.issubset(users.columns) and {"fitment_lat","fitment_lon"}.issubset(b.columns):
        u = users.dropna(subset=["base_lat","base_lon"]).copy()
        u["marker"] = "User"
        m = b.dropna(subset=["fitment_lat","fitment_lon"]).copy()
        m = m[m["risk_tier"]!="Low"]  # show Medium/High
        fig_umap = px.scatter_mapbox(
            m, lat="fitment_lat", lon="fitment_lon",
            color="risk_tier", size=np.clip(m["risk_score"]*20, 5, 25),
            hover_name="batch_id", zoom=4, height=520, title="At-Risk Fitments (Medium/High)"
        )
        # add users layer
        fig_umap.add_trace(px.scatter_mapbox(
            u, lat="base_lat", lon="base_lon", hover_name="name",
            color_discrete_sequence=["gray"]
        ).data[0])
        fig_umap.update_layout(mapbox_style="open-street-map")
        st.plotly_chart(fig_umap, use_container_width=True)
    else:
        st.info("Missing user base_lat/base_lon or batch fitment_lat/fitment_lon for coverage map.")

    # Staffing gaps: per state high-risk batches vs inspectors
    st.subheader("Staffing Gaps")
    inspectors = users[users.get("user_role","").astype(str).str.lower()=="inspector"].copy()
    if "base_state" in users.columns:
        hi = b[b["risk_tier"]=="High"].copy()
        hireq = hi.merge(vendors[["vendor_id","state"]], on="vendor_id", how="left", suffixes=("","_vendor"))
        # Prefer fitment location state if present in fitment_location string; otherwise use vendor state
        hireq["state_for_calc"] = hireq["fitment_location"].fillna("").str.split(",").str[1].str.strip()
        hireq.loc[hireq["state_for_calc"].isna() | (hireq["state_for_calc"]==""), "state_for_calc"] = hireq["state"]
        risk_by_state = hireq.groupby("state_for_calc").size().reset_index(name="high_risk_batches")

        insp_by_state = inspectors.groupby("base_state").size().reset_index(name="inspectors")
        gap_df = risk_by_state.merge(insp_by_state, left_on="state_for_calc", right_on="base_state", how="left")
        gap_df["inspectors"] = gap_df["inspectors"].fillna(0).astype(int)
        gap_df["ratio_risk_per_inspector"] = np.where(gap_df["inspectors"]>0,
                                                      gap_df["high_risk_batches"]/gap_df["inspectors"],
                                                      np.inf)
        st.dataframe(gap_df.sort_values(["ratio_risk_per_inspector","high_risk_batches"], ascending=[False, False]))
    else:
        st.info("users.csv missing 'base_state' for staffing gap analysis.")

    # Dispatch recommender: nearest inspectors to top clusters
    st.subheader("Dispatch Recommender (Nearest Inspectors to Hotspots)")
    if not clusters.empty and not inspectors.empty and {"base_lat","base_lon"}.issubset(inspectors.columns):
        top_clusters = clusters.head(3)
        rec_blocks = []
        for _, row in top_clusters.iterrows():
            clat, clon = row["centroid_lat"], row["centroid_lon"]
            top3 = nearest_k(clat, clon, inspectors.dropna(subset=["base_lat","base_lon"]), "base_lat", "base_lon", k=3)
            top3 = top3[["name","phone_number","base_city","base_state","base_lat","base_lon"]].copy()
            top3["cluster"] = int(row["cluster"])
            top3["hotspot_lat"] = float(clat)
            top3["hotspot_lon"] = float(clon)
            rec_blocks.append(top3)
        rec_df = pd.concat(rec_blocks, ignore_index=True) if rec_blocks else pd.DataFrame()
        st.dataframe(rec_df)
    else:
        st.info("Insufficient hotspot clusters or inspector locations to generate dispatch recommendations.")

# ========================== BATCHES TAB ==========================
with tab_batch:
    st.header("Batch Insights – Safety & Operations")

    # KPIs
    total_batches = len(b)
    high_risk = (b["risk_tier"]=="High").sum()
    expiring_90 = (b["days_to_expiry"] <= 90).sum()
    qc_leakage = ((b["qc_penalty"] > 0) & b["fitment_date"].notna()).sum() if "fitment_date" in b.columns else 0

    c1,c2,c3,c4 = st.columns(4)
    c1.metric("Total Batches", total_batches)
    c2.metric("High-Risk Assets", int(high_risk))
    c3.metric("Expiring ≤ 90 days", int(expiring_90))
    c4.metric("QC Leakage (installed & not 'Pass')", int(qc_leakage))

    # Expiry Watch
    st.subheader("⏳ Expiry Watch")
    bins = pd.cut(b["days_to_expiry"], bins=[-1e9,-1,30,60,90,180,1e9],
                  labels=["Expired","≤30","31–60","61–90","91–180",">180"])
    expiry_counts = b.groupby(bins, dropna=False).size().reset_index(name="count")
    fig_exp = px.bar(expiry_counts, x="days_to_expiry", y="count", title="Batches by Time to Expiry")
    st.plotly_chart(fig_exp, use_container_width=True)

    # Risk Distribution
    st.subheader("🔥 Risk Distribution")
    risk_counts = b["risk_tier"].value_counts().reset_index()
    risk_counts.columns = ["risk_tier","count"]
    fig_risk = px.bar(risk_counts, x="risk_tier", y="count", title="Risk Tier Counts")
    st.plotly_chart(fig_risk, use_container_width=True)

    # Map of At-Risk
    st.subheader("🧭 At-Risk Map")
    if {"fitment_lat","fitment_lon"}.issubset(b.columns):
        map_df = b.dropna(subset=["fitment_lat","fitment_lon"]).copy()
        map_df["bubble"] = np.clip(map_df["risk_score"]*20, 5, 25)
        fig_map = px.scatter_mapbox(
            map_df, lat="fitment_lat", lon="fitment_lon",
            color="risk_tier", size="bubble",
            hover_name="batch_id", hover_data=["vendor_id","fitment_location","risk_score"],
            zoom=4, height=520
        )
        fig_map.update_layout(mapbox_style="open-street-map")
        st.plotly_chart(fig_map, use_container_width=True)
    else:
        st.info("No lat/lon in batches for map.")

    # QC Leakage Table
    st.subheader("🧪 QC Leakage")
    if "fitment_date" in b.columns:
        leakage_df = b[(b["qc_penalty"] > 0) & (b["fitment_date"].notna())].copy()
        st.write(f"Found **{len(leakage_df)}** potential leakage records.")
        st.dataframe(leakage_df[["batch_id","vendor_id","qc_status","fitment_date","fitment_location"]])
    else:
        leakage_df = pd.DataFrame()
        st.info("No 'fitment_date' column to evaluate QC leakage.")

    # Inspection SLA
    st.subheader("🧰 Inspection SLA")
    b["overdue_flag"] = b["days_since_last_inspection"] > sla_days
    sla_counts = b["overdue_flag"].value_counts(dropna=False).rename({True:"Overdue", False:"Within SLA"})
    fig_sla = px.pie(values=sla_counts.values, names=sla_counts.index, title="Inspection Status")
    st.plotly_chart(fig_sla, use_container_width=True)

    # Replacement Plan
    st.subheader(f"🚨 Replacement Plan (Risk ≥ {replacement_risk_threshold})")
    replace_df = b[b["risk_score"] >= replacement_risk_threshold].sort_values("risk_score", ascending=False)
    st.write(f"Recommended replacements: **{len(replace_df)}**")
    st.dataframe(replace_df[["batch_id","vendor_id","fitment_location","risk_score","risk_tier","days_to_expiry"]])

    # Counterfeit / Duplicate QR
    st.subheader("🔒 Counterfeit & Duplicate QR Intelligence")
    dup_groups = b.groupby("qr_hash").agg(records=("batch_id","count")).reset_index()
    dup_hashes = dup_groups[dup_groups["records"]>1]["qr_hash"].tolist()

    suspicious = []
    if len(dup_hashes) > 0 and {"fitment_lat","fitment_lon"}.issubset(b.columns):
        for h in dup_hashes:
            sub = b[b["qr_hash"]==h].dropna(subset=["fitment_lat","fitment_lon"])
            for i in range(len(sub)):
                for j in range(i+1, len(sub)):
                    d = haversine_km(sub.iloc[i]["fitment_lat"], sub.iloc[i]["fitment_lon"],
                                     sub.iloc[j]["fitment_lat"], sub.iloc[j]["fitment_lon"])
                    if d > qr_anomaly_distance_km:
                        suspicious.append({
                            "qr_hash": h,
                            "batch_id_1": sub.iloc[i]["batch_id"],
                            "batch_id_2": sub.iloc[j]["batch_id"],
                            "distance_km": round(float(d), 2)
                        })
    st.write(f"Suspected duplicate-QR events: **{len(suspicious)}**")
    if suspicious:
        st.dataframe(pd.DataFrame(suspicious))

    # Exports
    st.subheader("⬇️ Exports")
    def download(df, name):
        st.download_button(
            f"Download {name} CSV",
            data=df.to_csv(index=False).encode("utf-8"),
            file_name=f"{name}.csv",
            mime="text/csv",
            use_container_width=True
        )
    download(b.sort_values("risk_score", ascending=False), "risk_table")
    download(replace_df, "replacement_plan")
    download(leakage_df, "qc_leakage")

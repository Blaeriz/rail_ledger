<script>
  export let data = [];
  export let columns = [];
  export let title = '';
  export let limit = 10;
  export let searchable = false;
  export let className = '';

  let searchTerm = '';
  let filteredData = data;

  $: {
    if (searchable && searchTerm) {
      filteredData = data.filter(item => 
        columns.some(col => 
          String(item[col.key] || '').toLowerCase().includes(searchTerm.toLowerCase())
        )
      );
    } else {
      filteredData = data;
    }
  }

  $: displayData = filteredData.slice(0, limit);
</script>

<div class="data-table-container {className}">
  {#if title}
    <div class="table-header">
      <h3>{title}</h3>
      {#if searchable}
        <input 
          type="text" 
          placeholder="Search..." 
          bind:value={searchTerm}
          class="search-input"
        />
      {/if}
    </div>
  {/if}
  
  <div class="table-wrapper">
    <table class="data-table">
      <thead>
        <tr>
          {#each columns as col}
            <th>{col.label}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each displayData as item, index}
          <tr>
            {#each columns as col}
              <td>
                {#if col.type === 'status'}
                  <span class="status-badge {item[col.key]?.toLowerCase().replace(' ', '-')}">
                    {item[col.key] || 'Unknown'}
                  </span>
                {:else if col.type === 'progress'}
                  <div class="progress-bar">
                    <div class="progress-fill" style="width: {item[col.key]}%"></div>
                    <span class="progress-text">{item[col.key]}%</span>
                  </div>
                {:else if col.type === 'date'}
                  {item[col.key] ? new Date(item[col.key]).toLocaleDateString() : 'N/A'}
                {:else}
                  {item[col.key] || 'N/A'}
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  
  {#if data.length > limit}
    <div class="table-footer">
      Showing {displayData.length} of {filteredData.length} results
    </div>
  {/if}
</div>

<style>
  .data-table-container {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .table-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #e5e7eb;
    background: #f8fafc;
  }

  .table-header h3 {
    margin: 0;
    color: #000000;
    font-size: 1.125rem;
    font-weight: 600;
  }

  .search-input {
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 0.875rem;
    width: 200px;
  }

  .table-wrapper {
    overflow-x: auto;
  }

  .data-table {
    width: 100%;
    border-collapse: collapse;
  }

  .data-table th {
    background: #f8fafc;
    color: #374151;
    font-weight: 600;
    padding: 0.75rem 1rem;
    text-align: left;
    border-bottom: 1px solid #e5e7eb;
    font-size: 0.875rem;
  }

  .data-table td {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #f3f4f6;
    color: #374151;
    font-size: 0.875rem;
  }

  .data-table tr:hover {
    background: #f9fafb;
  }

  .status-badge {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-badge.pass, .status-badge.completed {
    background: #dcfce7;
    color: #166534;
  }

  .status-badge.fail, .status-badge.failed {
    background: #fef2f2;
    color: #dc2626;
  }

  .status-badge.pending, .status-badge.pending-inspection {
    background: #fef3c7;
    color: #d97706;
  }

  .status-badge.unknown {
    background: #f3f4f6;
    color: #6b7280;
  }

  .progress-bar {
    position: relative;
    background: #e5e7eb;
    border-radius: 9999px;
    height: 6px;
    width: 80px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #10b981 0%, #059669 100%);
    border-radius: 9999px;
    transition: width 0.3s ease;
  }

  .progress-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 0.75rem;
    font-weight: 600;
    color: #374151;
  }

  .table-footer {
    padding: 0.75rem 1.5rem;
    background: #f8fafc;
    color: #6b7280;
    font-size: 0.875rem;
    text-align: center;
  }

  @media (max-width: 768px) {
    .table-header {
      flex-direction: column;
      gap: 1rem;
      align-items: stretch;
    }

    .search-input {
      width: 100%;
    }
  }
</style>

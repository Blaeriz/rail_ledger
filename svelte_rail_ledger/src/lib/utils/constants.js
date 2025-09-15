// Application constants
export const ROLES = {
  ADMIN: 'Admin',
  INSPECTOR: 'Inspector',
  VIEWER: 'Viewer'
};

export const ROUTES = {
  HOME: '/home',
  LOGIN: '/login',
  ADMIN_DASHBOARD: '/admin/dashboard',
  INSPECTOR_DASHBOARD: '/inspector/dashboard',
  VIEWER_DASHBOARD: '/viewer/dashboard',
  PROFILE: '/profile',
  REPORTS: '/reports'
};

export const DEMO_CREDENTIALS = [
  { username: 'admin', password: 'admin123', role: ROLES.ADMIN },
  { username: 'inspector', password: 'insp123', role: ROLES.INSPECTOR },
  { username: 'viewer', password: 'view123', role: ROLES.VIEWER }
];

export const QC_STATUS = {
  PENDING: 'Pending Inspection',
  PASS: 'Pass',
  FAIL: 'Fail'
};

export const FITMENT_STATUS = {
  NOT_STARTED: 'Not Started',
  IN_PROGRESS: 'In Progress',
  COMPLETED: 'Completed'
};

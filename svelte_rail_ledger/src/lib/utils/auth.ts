// Authentication utilities
export function isLoggedIn() {
  if (typeof window === 'undefined') return false;
  return localStorage.getItem('isLoggedIn') === 'true';
}

export function getUserRole() {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem('role');
}

export function getUsername() {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem('username');
}

export function login(username: string, role: string) {
  if (typeof window === 'undefined') return;
  localStorage.setItem('isLoggedIn', 'true');
  localStorage.setItem('username', username);
  localStorage.setItem('role', role);
}

export function logout() {
  if (typeof window === 'undefined') return;
  localStorage.removeItem('isLoggedIn');
  localStorage.removeItem('username');
  localStorage.removeItem('role');
}

export function getRedirectPath(role: string) {
  switch (role) {
    case 'Admin':
      return '/admin/dashboard';
    case 'Inspector':
      return '/inspector/dashboard';
    case 'Viewer':
      return '/viewer/dashboard';
    default:
      return '/home';
  }
}

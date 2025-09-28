import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { user_info } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';
import { logEvent } from '$lib/eventLog';

// ✅ GET a single user
export const GET: RequestHandler = async ({ params }) => {
  try {
    const id = params.id; // user_id is text
    if (!id) {
      logEvent('/api/users', 'GET', 'error');  // 👈 Correctly logs GET
      return new Response(JSON.stringify({ error: 'User ID is required' }), { status: 400 });
    }

    const [user] = await db.select().from(user_info).where(eq(user_info.user_id, id));
    if (!user) {
      logEvent('/api/users', 'GET', 'error');  // 👈 Correctly logs GET
      return new Response(JSON.stringify({ error: 'User not found' }), { status: 404 });
    }

    logEvent('/api/users', 'GET', 'success');  // 👈 Correctly logs GET
    return new Response(JSON.stringify(user), { status: 200 });
  } catch (err) {
    console.error('Error fetching user:', err);
    logEvent('/api/users', 'GET', 'error');    // 👈 Correctly logs GET
    return new Response(JSON.stringify({ error: 'Failed to fetch user' }), { status: 500 });
  }
};

// ✅ POST to create a new user
export const POST: RequestHandler = async ({ request }) => {
  try {
    const data = await request.json();
    const [inserted] = await db.insert(user_info).values({
      user_id: data.user_id,
      name: data.name,
      phone_number: data.phone_number,
      aadhar: data.aadhar,
      user_role: data.user_role
    }).returning();

    logEvent('/api/users', 'POST', 'success');  // 👈 Correctly logs POST
    return new Response(JSON.stringify(inserted), { status: 201 });
  } catch (err) {
    console.error('Error creating user:', err);
    logEvent('/api/users', 'POST', 'error');    // 👈 Correctly logs POST
    return new Response(JSON.stringify({ error: 'Failed to create user' }), { status: 500 });
  }
};

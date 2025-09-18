import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { user_info } from '$lib/server/db/schema';

// GET all users
export const GET: RequestHandler = async () => {
  try {
    const result = await db.select().from(user_info);
    return new Response(JSON.stringify(result), { status: 200 });
  } catch (err) {
    console.error('Error fetching users:', err);
    return new Response(JSON.stringify({ error: 'Failed to fetch users' }), { status: 500 });
  }
};

// POST a new user
export const POST: RequestHandler = async ({ request }) => {
  try {
    const data = await request.json();
    await db.insert(user_info).values({
      user_id: data.user_id,
      aadhar: data.aadhar,
      phone_number: data.phone_number,
      name: data.name,
      user_role: data.user_role
    });
    return new Response(JSON.stringify({ message: 'User added successfully' }), { status: 201 });
  } catch (err) {
    console.error('Error adding user:', err);
    return new Response(JSON.stringify({ error: 'Failed to add user' }), { status: 500 });
  }
};

import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { user_info } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';

// GET a single user
export const GET: RequestHandler = async ({ params }) => {
  try {
    const id = params.id; // user_id is text
    if (!id) {
      return new Response(JSON.stringify({ error: 'User ID is required' }), { status: 400 });
    }
    const [user] = await db.select().from(user_info).where(eq(user_info.user_id, id));
    if (!user) {
      return new Response(JSON.stringify({ error: 'User not found' }), { status: 404 });
    }
    return new Response(JSON.stringify(user), { status: 200 });
  } catch (err) {
    console.error('Error fetching user:', err);
    return new Response(JSON.stringify({ error: 'Failed to fetch user' }), { status: 500 });
  }
};

// UPDATE a user
export const PUT: RequestHandler = async ({ params, request }) => {
  try {
    const id = params.id;
    if (!id) {
      return new Response(JSON.stringify({ error: 'User ID is required' }), { status: 400 });
    }
    const data = await request.json();
    const [updated] = await db
      .update(user_info)
      .set({
        aadhar: data.aadhar,
        phone_number: data.phone_number,
        name: data.name,
        user_role: data.user_role
      })
      .where(eq(user_info.user_id, id))
      .returning();

    if (!updated) {
      return new Response(JSON.stringify({ error: 'User not found' }), { status: 404 });
    }
    return new Response(JSON.stringify(updated), { status: 200 });
  } catch (err) {
    console.error('Error updating user:', err);
    return new Response(JSON.stringify({ error: 'Failed to update user' }), { status: 500 });
  }
};

// DELETE a user
export const DELETE: RequestHandler = async ({ params }) => {
  try {
    const id = params.id;
    if (!id) {
      return new Response(JSON.stringify({ error: 'User ID is required' }), { status: 400 });
    }
    const [deleted] = await db.delete(user_info).where(eq(user_info.user_id, id)).returning();
    if (!deleted) {
      return new Response(JSON.stringify({ error: 'User not found' }), { status: 404 });
    }
    return new Response(JSON.stringify({ message: 'User deleted successfully' }), { status: 200 });
  } catch (err) {
    console.error('Error deleting user:', err);
    return new Response(JSON.stringify({ error: 'Failed to delete user' }), { status: 500 });
  }
};

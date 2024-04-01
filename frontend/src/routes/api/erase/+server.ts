import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export async function POST({ request }: { request: RequestHandler }) {
	const image = await request.json();

	const response = await fetch('http://localhost:5555/upload', {
		method: 'POST',
		body: JSON.stringify({
			image: image
		}),
		headers: {
			'content-type': 'application/json'
		}
	})
		.then((response: any) => response.json())
		.catch(() => {
			return {
				cropped_image: '',
				error_code: 90
			};
		});

	return json({ croppedImage: response.cropped_image, errorCode: response.error_code });
}

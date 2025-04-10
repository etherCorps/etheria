import type {PageLoad} from "./$types";
import {redirect} from "@sveltejs/kit";

export const load: PageLoad = async ({parent}) => {
    const data = await parent();
    if (data.isColimaRunning) {
        redirect(307, '/containers')
    }
    return data
}

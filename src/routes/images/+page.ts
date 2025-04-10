import type {PageLoad} from "./$types";
import {connectToDocker} from "$lib/invoker/colima";
import {redirect} from "@sveltejs/kit";
import {getImages} from "$lib/invoker/images";

export const load: PageLoad = async ({parent}) => {
    const data = await parent();
    if (!data.isColimaRunning || !data.isColimaInstalled || !data.isDockerInstalled) {
        redirect(302, '/');
    }

    if (data.isColimaRunning) {
        await connectToDocker()
    }

    return {
        images: await getImages(true)
    }
}

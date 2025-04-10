import type {PageLoad} from "./$types";
import {getContainers} from "$lib/invoker/container";
import {connectToDocker} from "$lib/invoker/colima";
import {redirect} from "@sveltejs/kit";

export const load: PageLoad = async ({parent}) => {
    const data = await parent();
    if (!data.isColimaRunning || !data.isColimaInstalled || !data.isDockerInstalled) {
        redirect(302, '/');
    }
    if (data.isColimaRunning) {
        await connectToDocker()
    }

    return {
        containers: await getContainers()
    }
}

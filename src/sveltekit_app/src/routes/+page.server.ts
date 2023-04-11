import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async (req) => {
    return {
        data: "test hahahaha",
    }
}
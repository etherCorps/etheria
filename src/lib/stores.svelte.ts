export type PageState = {
    isLoading: boolean;
}

export const pageState: PageState = $state({
    isLoading: false,
});
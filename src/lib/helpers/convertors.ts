export function kebabToNormal(kebabCaseString: string) {
    return kebabCaseString
        .toLowerCase()
        .replace(/-/g, ' ')
        .replace(/\b\w/g, (match) => match.toUpperCase()).replace('/', '');
}
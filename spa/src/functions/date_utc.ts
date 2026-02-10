export const get_utc_info = (): string => {
    const date: Date = new Date();
    return date.toUTCString()
}

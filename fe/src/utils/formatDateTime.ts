export function formatDateTime(date: Date): string {
    const options: Intl.DateTimeFormatOptions = {
        year: '2-digit',
        month: 'short', // Use '2-digit' for numeric months
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
    };

    return new Intl.DateTimeFormat('en-US', options).format(date);
}
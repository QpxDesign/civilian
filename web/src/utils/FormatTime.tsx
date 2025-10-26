export function DurationAgo(time_unix: number): String {
  const CURRENT_TIME = Math.floor(Date.now() / 1000);
  const delta = CURRENT_TIME - time_unix;
  if (delta > 60 * 60) {
    // (more than an hour)
    const a = Math.floor(delta / 60 / 60);
    if (a > 1) {
      return `${a} hours ago`;
    } else {
      return `${a} hour ago`;
    }
  } else {
    const a = Math.floor(delta / 60);
    return `${a} min ago`;
  }
}

export function Distance(
  x_1: number,
  y_1: number,
  x_2: number,
  y_2: number,
): String {
  const dist = Math.sqrt(Math.pow(x_2 - x_1, 2) + Math.pow(y_2 - y_1, 2)) * 69;
  return dist.toFixed(2);
}

import { AppError, Result } from '@backend';

export function unwrap<T, E>(result: Result<T, E>): T;

export function unwrap<T, E>(result: Promise<Result<T, E>>): Promise<T>;

export async function unwrap<T>(
  result: Result<T, AppError> | Promise<Result<T, AppError>>,
): Promise<T> {
  const resolved = await result;

  if (resolved.status === 'ok') {
    return resolved.data;
  }

  // todo: handle error with useNotification
  throw unreachable();
}

export function setHook() {
  window.addEventListener('unhandledrejection', (e) => {
    if (e.reason instanceof UnreachableError) {
      e.preventDefault();
      return;
    }
  });
}

export class UnreachableError extends Error {
  readonly kind = 'unreachable';
}

export function unreachable(): never {
  throw new UnreachableError();
}

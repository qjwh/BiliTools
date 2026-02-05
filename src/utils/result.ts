import { AppError, Result } from '@backend';
import { message } from '.';
import i18n from '@/i18n';

export function unwrap<T, E>(result: Result<T, E>): T | null;

export function unwrap<T, E>(result: Promise<Result<T, E>>): Promise<T | null>;

export async function unwrap<T>(
  result: Result<T, AppError> | Promise<Result<T, AppError>>,
): Promise<T | null> {
  const resolved = await result;

  if (resolved.status === 'ok') {
    return resolved.data;
  }

  const errPath = `error.${resolved.error.source}.${resolved.error.reason}`;

  const err = i18n.global.te(errPath) ? i18n.global.t(errPath).split('\n') : [errPath];

  message.notify({
    title: err[0],
    body: err[1],
    type: 'error',
  });

  return null;
}

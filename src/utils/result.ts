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

  const errRoot = `error.${resolved.error.source}.${resolved.error.reason}`;

  message.notify({
    title: i18n.global.t(`${errRoot}.title`),
    body: i18n.global.t(`${errRoot}.body`),
    type: 'error',
  });

  return null;
}

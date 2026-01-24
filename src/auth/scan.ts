import { result } from '@/utils';
import { commands } from '@backend';

export async function prepare() {
  return await result.unwrap(commands.scanLoginPrepare());
}

export async function poll(qrcodeKey: string) {
  return await result.unwrap(commands.scanLogin(qrcodeKey));
}

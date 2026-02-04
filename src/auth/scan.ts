import { result } from '@/utils';
import { commands } from '@backend';

export async function genQrCode() {
  return await result.unwrap(commands.scanLoginGenQrcode());
}

export async function poll(qrcodeKey: string) {
  return await result.unwrap(commands.scanLogin(qrcodeKey));
}

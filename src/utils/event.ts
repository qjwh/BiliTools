import { sign } from '@/auth';
import { events, FrontendRequestPayload } from '@backend';
import { getCurrentWindow } from '@tauri-apps/api/window';

export function setHook() {
  events.frontendRequest.listen(async (event) => {
    const { id, payload } = event.payload;
    const result = await handleEvent(payload);
    await getCurrentWindow().emit(id, result);
  });
}

async function handleEvent(payload: FrontendRequestPayload) {
  switch (payload.type) {
    case 'BiliUuid':
      return sign.biliUuid(payload.now);
    case 'BiliTicketHexSign':
      return sign.biliTicketHexSign(payload.message, payload.key);
  }
}

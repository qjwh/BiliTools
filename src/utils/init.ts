import { commands, InitEvent } from '@backend';
import { Channel } from '@tauri-apps/api/core';

import * as auth from '@/auth';
import { result, event } from '.';

export async function init() {
  event.setHook();

  const initChannel = new Channel<InitEvent>();
  initChannel.onmessage = (event) => {
    switch (event.type) {
      case 'result':
        return result.unwrap(event.data);
    }
  };
  result.unwrap(commands.init(initChannel));
  result.unwrap(commands.ensureRequestEnv());

  await auth.profile.getProfile();
}

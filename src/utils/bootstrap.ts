import { Channel } from "@tauri-apps/api/core";
import { commands, InitEvent } from "@backend";

import { useSystemStore } from "@/store";
import * as auth from '@/auth';
import * as ipc from '@/ipc';

import { result } from ".";

export async function bootstrap(opt: {
  mount: () => void,
}) {
  const config = await commands.resolveConfig();
  const system = useSystemStore();
  system.apply(config.locale, config.theme, config.window_effect);

  // Before this, UI is not rendered
  opt.mount();
  // After this, UI should be ready

  ipc.setHook();
  const initChannel = new Channel<InitEvent>();
  initChannel.onmessage = (event) => {
    switch(event.type) {
      case 'result':
        return result.unwrap(event.data);
    }
  };
  result.unwrap(commands.init(initChannel));
  result.unwrap(commands.ensureRequestEnv());

  await auth.profile.getProfile();
}

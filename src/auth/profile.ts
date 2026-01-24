import { Profile } from '@/store/auth';
import { useAuthStore } from '@/store';

import { auth as api } from '@/api';

export async function getProfile() {
  const auth = useAuthStore();

  const nav = (await api.getNav()).data;
  const navStat = (await api.getNavStat()).data;

  if (!nav.isLogin) {
    // use `as` to avoid the prefer-spread lint
    auth.apply(null as null, 'anonymous');
    return;
  }

  const profile: Profile = {
    avatar: nav.face,
    name: nav.uname,
    isVip: nav.vipStatus !== 0,
    level: nav.level_info.current_level,
    uid: nav.mid,
    coins: nav.money,
    bcoin: nav.wallet.bcoin_balance,
    following: navStat.following,
    follower: navStat.follower,
    dynamic_count: navStat.dynamic_count,
  };

  auth.apply(profile, 'valid');
}

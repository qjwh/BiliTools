import { defineStore } from 'pinia';

export type AuthStatus = 'checking' | 'anonymous' | 'valid';

export interface Profile {
  avatar: string;
  name: string;
  isVip: boolean;
  level: number;
  uid: number;
  coins: number;
  bcoin: number;
  following: number;
  follower: number;
  dynamic_count: number;
}

export const useAuthStore = defineStore('auth', {
  state: (): {
    status: AuthStatus;
    profile: Profile | null;
  } => ({
    status: 'checking',
    profile: null,
  }),
  getters: {
    isChecking: (s) => s.status === 'checking',
    isAnonymous: (s) => s.status === 'anonymous',
    isValid: (s) => s.status === 'valid',
  },
  actions: {
    apply(profile: Profile | null, status: AuthStatus) {
      this.status = status;
      this.profile = profile;
    },
  },
});

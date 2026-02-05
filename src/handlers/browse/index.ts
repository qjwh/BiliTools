export enum BrowseKind {
  Video = 'video',
  Bangumi = 'bangumi',
  Cheese = 'cheese',
  Audio = 'audio',
  AudioList = 'audioList',
  Read = 'read',
  ReadList = 'readList',
  User = 'user',
  Watchlater = 'watchlater',
}

export interface BrowseHandler {
  readonly kind: BrowseKind,

  load(): Promise<void>,
}

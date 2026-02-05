export interface NavResponse {
  code: number;
  message: string;
  ttl: number;
  data: {
    isLogin: boolean;
    face: string;
    level_info: {
      current_level: number;
    };
    mid: number;
    money: number;
    uname: string;
    vipStatus: number;
    vip_label: {
      img_label_uri_hans_static: string;
      img_label_uri_hant_static: string;
    };
    wallet: {
      bcoin_balance: number;
    };
    wbi_img: {
      img_url: string;
      sub_url: string;
    };
  };
}

export interface NavStatResponse {
  code: number;
  message: string;
  ttl: number;
  data: {
    following: number;
    follower: number;
    dynamic_count: number;
  };
}

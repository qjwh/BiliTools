import { GlobalThemeOverrides } from 'naive-ui';

const common: GlobalThemeOverrides['common'] = {
  primaryColor: '#0087BD',
  primaryColorHover: '#58B1D4',
  primaryColorPressed: '#2C9CC8',
  primaryColorSuppl: '#006996',
};

export const lightThemeOverrides: GlobalThemeOverrides = {
  common: {
    ...common,
  },
};

export const darkThemeOverrides: GlobalThemeOverrides = {
  common: {
    ...common,
  },
  Alert: {
    color: 'rgb(40, 40, 44)',
    colorInfo: 'rgb(26, 46, 64)',
    colorSuccess: 'rgb(22, 49, 46)',
    colorWarning: 'rgb(72, 46 ,15)',
    colorError: 'rgb(64, 26, 36)',
  }
};

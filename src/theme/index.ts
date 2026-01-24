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
};

import { defineConfig } from 'eslint/config';

import eslintPluginVue from 'eslint-plugin-vue';
import eslintConfigPrettier from 'eslint-config-prettier/flat';
import typescriptEslint from 'typescript-eslint';
import vueEslintParser from 'vue-eslint-parser';

export default defineConfig([
  {
    ignores: ['dist', 'node_modules', 'generated', 'public'],
  },
  // Typescript
  typescriptEslint.configs.recommended,
  {
    files: ['**/*.ts'],
    rules: {
      eqeqeq: ['error'],
    },
  },
  // Vue SFC
  eslintPluginVue.configs['flat/recommended'],
  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: vueEslintParser,
      parserOptions: {
        parser: typescriptEslint.parser,
      },
    },
    rules: {
      'vue/multi-word-component-names': 'off',
      eqeqeq: ['error'],
    },
  },
  eslintConfigPrettier,
]);

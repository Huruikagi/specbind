export const supportedLanguages = ['en', 'ja'] as const;

export type SupportedLanguage = (typeof supportedLanguages)[number];

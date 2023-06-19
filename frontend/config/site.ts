export type SiteConfig = typeof siteConfig;

export const siteConfig = {
  name: "MarkUp AI",
  description:
    "Beautifully designed components built with Radix UI and Tailwind CSS.",
  mainNav: [
    {
      title: "Grading",
      href: "/grade",
    },
    {
      title: "Generate",
      href: "/generate",
    },
    {
      title: "Question Gallery",
      href: "/gallery",
    },
  ],
  links: {
    github: "https://github.com/marcuspang/markup",
    docs: "https://ui.shadcn.com",
  },
};

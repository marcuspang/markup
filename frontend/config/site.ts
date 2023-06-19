export type SiteConfig = typeof siteConfig;

export const siteConfig = {
  name: "Next.js",
  description:
    "Beautifully designed components built with Radix UI and Tailwind CSS.",
  mainNav: [
    {
      title: "Home",
      href: "/",
    },
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

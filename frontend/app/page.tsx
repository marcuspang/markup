import Link from "next/link";

import { buttonVariants } from "@/components/ui/button";

export default function IndexPage() {
  return (
    <section className="container grid items-center gap-6 pb-8 pt-6 md:py-10">
      <div className="flex max-w-[980px] flex-col items-start gap-2">
        <h1 className="text-3xl font-extrabold leading-tight tracking-tighter md:text-4xl">
          MarkUp AI
        </h1>
        <p className="text-muted-foreground max-w-[700px] text-lg">
          Accessible and customizable question generation tailored for your
          students.
          <br className="hidden sm:inline" /> Free. Open Source. And Secure.
        </p>
      </div>
      <div className="flex gap-4">
        <Link href={"/grade"} className={buttonVariants()}>
          Grade your scripts
        </Link>
        <Link
          href={"/generate"}
          className={buttonVariants({ variant: "outline" })}
        >
          Generate sample questions
        </Link>
      </div>
    </section>
  );
}

"use client";

import { Loader2 } from "lucide-react";
import { signIn, signOut, useSession } from "next-auth/react";

import { Button } from "./ui/button";

export function AuthButton() {
  const { status } = useSession();

  if (status === "loading") {
    return (
      <Button disabled>
        <Loader2 className="repeat-infinite animate-spin" />
      </Button>
    );
  }

  if (status === "authenticated") {
    return <Button onClick={() => signOut()}>Logout</Button>;
  }

  return <Button onClick={() => signIn()}>Login</Button>;
}

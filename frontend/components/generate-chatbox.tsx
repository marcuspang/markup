import { Label } from "@radix-ui/react-label";
import { useChat } from "ai/react";

import { Input } from "./ui/input";

const messages = [
  {
    content: "hello",
    role: "user",
    createdAt: "2023-06-19T16:45:04.949Z",
    id: "4WK27L7",
  },
  {
    id: "vKStbLh",
    createdAt: "2023-06-19T16:45:06.096Z",
    content: "Hello! How can I assist you today?",
    role: "assistant",
  },
];

export function GenerateChatbox() {
  const { input, handleInputChange, handleSubmit } = useChat();

  return (
    <div className="flex space-x-2">
      <div className="w-1/3">
        <form onSubmit={handleSubmit}>
          <Label htmlFor="prompt">Enter your prompt</Label>
          <Input
            className="mb-4 w-full rounded border p-2"
            id="prompt"
            value={input}
            placeholder="Enter your topic here"
            onChange={handleInputChange}
          />
          <Label htmlFor="questions">Upload sample questions (PDF)</Label>
          <Input
            id="questions"
            type="file"
            accept="application/pdf"
            className="mt-2"
          />
        </form>
      </div>
      <div className="w-2/3 space-y-2 rounded border p-4">
        {messages.length > 0
          ? messages.map((m) => (
              <div
                key={m.id}
                className="whitespace-pre-wrap rounded border bg-gray-100 p-2 dark:bg-gray-800"
              >
                {m.role === "user" ? "User: " : "AI: "}
                {m.content}
              </div>
            ))
          : null}
      </div>
    </div>
  );
}

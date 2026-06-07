// TypeScript has a real `enum`, but usually better to use a union of string literals.
// It has no runtime cost and prints as itself.
type Status = "pending" | "running" | "succeeded" | "failed";

// A discriminated (tagged) union: every variant shares a literal `kind` field,
// which TypeScript uses to narrow the type in each branch.
type Media =
  | { kind: "text"; length: number }
  | { kind: "image"; width: number; height: number }
  | { kind: "video"; width: number; height: number; duration: number };

function describe(m: Media): string {
  switch (m.kind) {
    case "text":
      return `text: ${m.length} chars`;
    case "image":
      return `image: ${m.width}x${m.height}`;
    case "video":
      return `video: ${m.width}x${m.height}, ${m.duration}s`;
    default: {
      // Exhaustiveness check: if a variant is added but not handled,
      // this line stops compiling because `m` is no longer `never`.
      const _exhaustive: never = m;
      return _exhaustive;
    }
  }
}

function main() {
  const status: Status = "running";
  console.log(`status: ${status}`);

  const items: Media[] = [
    { kind: "text", length: 280 },
    { kind: "image", width: 1920, height: 1080 },
    { kind: "video", width: 1920, height: 1080, duration: 12.5 },
  ];
  for (const m of items) {
    console.log(describe(m));
  }
}

main();

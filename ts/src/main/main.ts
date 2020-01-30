async function main(): Promise<void> {
  console.error("Not implemented");
  return;
}

main()
  .catch((err: Error): never => {
    console.error(err.stack);
    process.exit(1);
    return undefined as never;
  });

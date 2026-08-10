# Present so pytest puts packages/textkit on sys.path, making `import textkit`
# work with the command TASK.md prescribes. No fixtures: the seam under test is a
# pure function and needs no setup.

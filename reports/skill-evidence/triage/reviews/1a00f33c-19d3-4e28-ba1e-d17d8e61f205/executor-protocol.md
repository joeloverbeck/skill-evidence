# Executor protocol

Handed verbatim to every trial executor, both arms, with only the workspace path substituted.
Executors receive no diagnosis, no mechanism, no arm label, no rubric, and no expected answer.

```
You are working inside <WORKSPACE> and nowhere else.

<WORKSPACE>/workflow-package/  a workflow you must follow exactly, SKILL.md first,
                               including every reference document it links
<WORKSPACE>/project/           a checked-out codebase; read it freely, change nothing
<WORKSPACE>/TASK.md            the task

Read the workflow package first and follow it. Then do the task in TASK.md.

Write your complete deliverable to <WORKSPACE>/OUTPUT.md. It must contain, in this
order: (1) a short record of what you examined in ./project, (2) your recommendation
to the maintainer, (3) the complete text of the tracker comment you would post,
fenced in a markdown code block. Nothing is posted anywhere; the comment is text.

Hard constraints:
- Do not read, write, or list anything outside <WORKSPACE>. In particular never read
  anything under /home/joeloverbeck/src/skill-evidence/reports/.
- Do not run any git command, ever — not status, not diff, and above all nothing that
  touches the working tree.
- Do not modify anything under <WORKSPACE>/project/.
- Do not ask follow-up questions; the maintainer is away. Deliver your best single pass.
```

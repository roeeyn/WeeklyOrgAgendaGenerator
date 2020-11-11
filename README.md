# Weekly Org Agenda Generator

This project is for automating the creation of the weekly schedule for recurring activities included in an org agenda. 

This was thought specifically for being used with Spacemacs, but I assume it can be used with Emacs too.

## Use 

You can use the release file from the repo and execute it. You need to give the next Monday date and the `tasks.json` file as arguments.

For example:

```bash
./woag 2020-11-09 /Users/roeeyn/tasks.json
```

An example of the `tasks.json` could be the following:

```json
[
  {
    "name": "FP Course",
    "schedule": "17:00-17:30"
  },
  {
    "name": "AWS Certification",
    "schedule": "14:30-15:00"
  },
  {
    "name": "Algorithms",
    "schedule": "14:00-14:30"
  }
]
```

The output of that example would be:

```
** Week 46

*** Monday 09/11/20

**** TODO FP Course
     SCHEDULED: <2020-11-09 Mon 17:00-17:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO AWS Certification
     SCHEDULED: <2020-11-09 Mon 14:30-15:00>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO Algorithms
     SCHEDULED: <2020-11-09 Mon 14:00-14:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

*** Tuesday 10/11/20

**** TODO FP Course
     SCHEDULED: <2020-11-10 Tue 17:00-17:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO AWS Certification
     SCHEDULED: <2020-11-10 Tue 14:30-15:00>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO Algorithms
     SCHEDULED: <2020-11-10 Tue 14:00-14:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

*** Wednesday 11/11/20

**** TODO FP Course
     SCHEDULED: <2020-11-11 Wed 17:00-17:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO AWS Certification
     SCHEDULED: <2020-11-11 Wed 14:30-15:00>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO Algorithms
     SCHEDULED: <2020-11-11 Wed 14:00-14:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

*** Thursday 12/11/20

**** TODO FP Course
     SCHEDULED: <2020-11-12 Thu 17:00-17:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO AWS Certification
     SCHEDULED: <2020-11-12 Thu 14:30-15:00>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO Algorithms
     SCHEDULED: <2020-11-12 Thu 14:00-14:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

*** Friday 13/11/20

**** TODO FP Course
     SCHEDULED: <2020-11-13 Fri 17:00-17:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO AWS Certification
     SCHEDULED: <2020-11-13 Fri 14:30-15:00>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO Algorithms
     SCHEDULED: <2020-11-13 Fri 14:00-14:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

*** Saturday 14/11/20

**** TODO FP Course
     SCHEDULED: <2020-11-14 Sat 17:00-17:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO AWS Certification
     SCHEDULED: <2020-11-14 Sat 14:30-15:00>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO Algorithms
     SCHEDULED: <2020-11-14 Sat 14:00-14:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

*** Sunday 15/11/20

**** TODO FP Course
     SCHEDULED: <2020-11-15 Sun 17:00-17:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO AWS Certification
     SCHEDULED: <2020-11-15 Sun 14:30-15:00>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:

**** TODO Algorithms
     SCHEDULED: <2020-11-15 Sun 14:00-14:30>
     :PROPERTIES:
       :WILD_NOTIFIER_NOTIFY_BEFORE: 5 0
     :END:
```

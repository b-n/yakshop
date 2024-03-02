# Notes from Yakshop

Some of the notes from the brief.

## Context

Business Owner: Yak shepherd - Brick and mortar, wants to expand online.
Customers: Yak wool and milk consumers.

## Yaks

Older => Less Milk.

Yak year: 100 days.

Yak lifespan = 10 years = 1000 days (dies on day 1000)

Production:

- D = age in days of the Yak
- Milk: 50 - D * 0.03
- Wool:
  - Every 8 + D * 0.01 can be shaven (on the day of shave)
  - Only after Year 1 (e.g. D >= 100)

Other notes:

- Shop opens on day 0
  - Wool can be shaved immediately (e.g. next shave moment is determined from current date)
- Every day (including day 0) yaks are milked + shaven (if elegible)
  - All yaks are shaven on day 0
  - Next date is calculated on the day of shaving (?)
  - If shave date is > current date, it needs to be next date. e.g. If day === 13 can be shaven on day 13. If 13.01, can only be shaven on day 14 (ceil the date of next shave)


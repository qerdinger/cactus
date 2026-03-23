import aiohttp
import asyncio
import time

TOTAL = 1000
CONCURRENCY = 800
URL = "http://localhost:8080/not_found?a={}"

inflight = 0
done = 0
errors = 0
start_time = time.time()


async def worker(sem, session, i):
    global inflight, done, errors

    async with sem:
        inflight += 1

        try:
            async with session.get(URL.format(i)) as resp:
                if 200 <= resp.status < 300:
                    pass
                else:
                    errors += 1
        except Exception:
            errors += 1

        inflight -= 1
        done += 1


async def reporter():
    while done < TOTAL:
        elapsed = time.time() - start_time
        rps = done / elapsed if elapsed > 0 else 0

        print(
            f"\rIn-flight: {inflight} | Done: {done} | Errors: {errors} | RPS: {rps:.1f}",
            end="",
            flush=True,
        )
        await asyncio.sleep(0.2)

    print()


async def main():
    sem = asyncio.Semaphore(CONCURRENCY)

    async with aiohttp.ClientSession() as session:
        tasks = [worker(sem, session, i) for i in range(1, TOTAL + 1)]
        await asyncio.gather(reporter(), *tasks)
    
    print("ended!")


asyncio.run(main())
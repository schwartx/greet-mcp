import asyncio

from greet_mcp import start_server


async def main():
    await start_server()


asyncio.run(main())

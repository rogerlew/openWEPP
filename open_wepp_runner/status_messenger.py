from __future__ import annotations

import os
from urllib.parse import urlparse


class StatusMessenger:
    """
    Publish optional run-status lines to Redis.

    This class intentionally requires the `redis` package when used. If Redis
    publishing is requested without the dependency, it fails explicitly.
    """

    _client = None
    _redis_config: dict[str, object] = {}

    @classmethod
    def _build_config(cls) -> dict[str, object]:
        redis_url = os.environ.get("REDIS_URL", "").strip()
        redis_host = os.environ.get("REDIS_HOST", "localhost").strip()
        redis_port = int(os.environ.get("REDIS_PORT", "6379"))
        redis_password = os.environ.get("REDIS_PASSWORD", "").strip()

        if redis_url:
            parsed = urlparse(redis_url)
            if parsed.scheme and parsed.hostname:
                config: dict[str, object] = {
                    "host": parsed.hostname,
                    "port": parsed.port or redis_port,
                    "db": int(parsed.path.lstrip("/") or "2"),
                    "decode_responses": True,
                }
                if parsed.password:
                    config["password"] = parsed.password
                elif redis_password:
                    config["password"] = redis_password
                return config

        config = {
            "host": redis_host,
            "port": redis_port,
            "db": 2,
            "decode_responses": True,
        }
        if redis_password:
            config["password"] = redis_password
        return config

    @classmethod
    def _get_client(cls):
        if cls._client is None:
            try:
                import redis  # type: ignore
            except ImportError as exc:
                raise RuntimeError(
                    "OPEN_RUNNER-E-200 redis package is required for status publishing"
                ) from exc
            if not cls._redis_config:
                cls._redis_config = cls._build_config()
            cls._client = redis.Redis(**cls._redis_config)
        return cls._client

    @classmethod
    def publish(cls, channel: str, message: str) -> int:
        if not channel:
            raise ValueError("OPEN_RUNNER-E-201 publish channel must be non-empty")
        return cls._get_client().publish(channel, message)

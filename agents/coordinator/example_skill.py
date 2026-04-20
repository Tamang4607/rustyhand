"""Distance between two GPS coordinates (Haversine formula).

Example skill that Capability Builder might generate when the user asks
something like "find the distance between Almaty and Astana".

Skills go to ~/.rustyhand/skills/ and are auto-loaded on hot-reload.
"""
import math


def run(input: dict) -> dict:
    """Compute great-circle distance in km between two (lat, lon) points.

    Args:
        input: dict with keys:
            - "from": [lat, lon] of point A
            - "to":   [lat, lon] of point B

    Returns:
        dict: {"ok": True, "distance_km": float} on success
              {"error": str} on validation failure
    """
    if "from" not in input or "to" not in input:
        return {"error": "missing 'from' or 'to' coordinate"}

    try:
        lat1, lon1 = float(input["from"][0]), float(input["from"][1])
        lat2, lon2 = float(input["to"][0]), float(input["to"][1])
    except (TypeError, ValueError, IndexError):
        return {"error": "coordinates must be [latitude, longitude] floats"}

    # Haversine formula
    R = 6371.0  # Earth radius in km
    phi1, phi2 = math.radians(lat1), math.radians(lat2)
    dphi = math.radians(lat2 - lat1)
    dlambda = math.radians(lon2 - lon1)

    a = (
        math.sin(dphi / 2) ** 2
        + math.cos(phi1) * math.cos(phi2) * math.sin(dlambda / 2) ** 2
    )
    distance = 2 * R * math.asin(math.sqrt(a))

    return {"ok": True, "distance_km": round(distance, 2)}


if __name__ == "__main__":
    # Self-test: Almaty (43.2389, 76.8897) → Astana (51.1605, 71.4704)
    result = run({"from": [43.2389, 76.8897], "to": [51.1605, 71.4704]})
    assert result["ok"], result
    assert 950 < result["distance_km"] < 1000, f"unexpected distance: {result}"
    print("OK:", result)

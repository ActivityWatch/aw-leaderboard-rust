from fastapi import APIRouter

router = APIRouter(prefix="/leaderboard", tags=["leaderboard"])


# Leaderboard routes
@router.get("/all-time")
async def get_all_time_leaderboard():
    # Your code to get all-time leaderboard
    pass


@router.get("/category/{category}")
async def get_category_leaderboard(category: str):
    # Your code to get leaderboard for a specific category
    pass

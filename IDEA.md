

enum Asset
{
    Content, // Non gameplay impacting element, such as texture, sound... Can be loaded async
    Resource, // Gameplay impacting element. Should be loaded/available at the same time for all players
}


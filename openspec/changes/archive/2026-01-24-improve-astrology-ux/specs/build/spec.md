# Build Spec Delta

## MODIFIED Requirements

### Requirement: Search Index Generation

The search index generator MUST include astrology content in addition to blog posts.

#### Scenario: Search index includes astrology items
- Given the astrology data contains 39 items across 4 categories
- When the search index is generated
- Then each astrology item is added to the search index with:
  - `slug`: the item's slug
  - `url`: the full URL path (e.g., "astrology/signs/aries/")
  - `title`: the item's title
  - `summary`: the item's summary or first 200 chars of content
  - `content`: the item's HTML content (stripped of tags, first 2000 chars)
  - `tags`: the item's keywords
  - `date`: empty string (astrology items have no date)
  - `item_type`: "astrology" to distinguish from posts
  - `category`: the category name (星座/行星/宫位/相位)

#### Scenario: Search index item type field
- Given the search index contains both posts and astrology items
- When a user searches for content
- Then each result includes an `item_type` field ("post" or "astrology")
- And an optional `category` field for astrology items

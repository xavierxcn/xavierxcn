# Render Spec Delta

## MODIFIED Requirements

### Requirement: Astrology Index Page Layout

The astrology index page MUST provide direct access to all items without requiring navigation through category pages.

#### Scenario: Compact hero section
- Given a user visits the astrology index page
- When the page loads
- Then the hero section displays a compact header with title only
- And the hero section takes minimal vertical space (no large decorative elements)

#### Scenario: Category tabs for filtering
- Given the astrology index page is loaded
- When the user views the page
- Then horizontal category tabs are displayed (全部/星座/行星/宫位/相位)
- And clicking a tab filters the displayed items to that category
- And the "全部" tab shows all 39 items

#### Scenario: Direct item grid display
- Given a user visits the astrology index page
- When the default "全部" tab is active
- Then all astrology items are displayed in a responsive grid
- And each item card shows: symbol, title, summary
- And clicking an item card navigates directly to the item page

#### Scenario: Smooth tab transitions
- Given the user clicks a different category tab
- When the tab becomes active
- Then items are filtered with a smooth fade/slide animation
- And the active tab is visually highlighted

### Requirement: Astrology Index Context

The template context MUST include all items for direct rendering on the index page.

#### Scenario: All items passed to template
- Given the astrology index template is being rendered
- When the context is prepared
- Then all astrology items from all categories are passed to the template
- And items are grouped by category for filtering

## MODIFIED Requirements

### Requirement: Search Results Display

The search results display MUST differentiate astrology content from blog posts.

#### Scenario: Astrology item in search results
- Given the search index contains astrology items
- When a user searches for "白羊座"
- Then the search results show matching astrology items
- And each result displays a category label (e.g., "星座")
- And clicking the result navigates to the astrology item page

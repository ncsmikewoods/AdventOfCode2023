namespace Day03;

public class Schematic
{
    public Schematic(List<List<char>> grid)
    {
        var labels = new List<Label>();

        for (var rowIndex = 0; rowIndex < grid.Count; rowIndex++)
        {
            var colStart = -1;
            var num = "";

            for (var colIndex = 0; colIndex < grid[rowIndex].Count; colIndex++)
            {
                var curr = grid[rowIndex][colIndex];
                
                // case: starting a new number or adding to an existing one
                if (char.IsNumber(curr))
                {
                    if (colStart == -1)
                    {
                        // new number starting
                        colStart = colIndex;
                        num += curr;
                    }
                    else
                    {
                        // adding a digit to an existing number
                        num += curr;                        
                    }
                }
                else
                {
                    if (colStart != -1)
                    {
                        // case: this element ends a number
                        var label = new Label
                        {
                            Row = rowIndex,
                            Col = colStart,
                            Length = num.Length,
                            Value = int.Parse(num)
                        };
                        
                        labels.Add(label);
                        colStart = -1;
                        num = "";
                    }
                }
            }
            
            // account for colStart not being reset which means a line ends with a number and never got terminated in the inner loop
            if (colStart != -1)
            {
                var labelTemp = new Label
                {
                    Row = rowIndex,
                    Col = colStart,
                    Length = num.Length,
                    Value = int.Parse(num)
                };
                        
                labels.Add(labelTemp);
            }
        }

        Grid = grid;
        Width = grid[0].Count;
        Height = grid.Count;
        Labels = labels;
    }
    
    public int Width { get; set; }
    public int Height { get; set; }
    public List<Label> Labels { get; set; }
    public List<List<char>> Grid { get; set; }
    
    public List<int> GetPartNumbers()
    {
        var partNumbers = new List<int>();

        foreach (var label in Labels)
        {
            var adjacent = GetAdjacent(label);
            if (adjacent.Any(c => !char.IsNumber(c) && c != '.'))
            {
                partNumbers.Add(label.Value);                
            }
        }

        return partNumbers;
    }
    
    List<char> GetAdjacent(Label label)
    {
        var adjacent = new List<char>();

        var leftBound = Math.Max(0, label.Col - 1);
        var rightBound = Math.Min(Width, label.Col + label.Length + 1);
        var rangeLength = rightBound - leftBound;
        
        // Console.WriteLine($"leftBound {leftBound}, rightBound {rightBound}");
        
        // get from row above
        if (label.Row != 0)
        {
            var adjacentAbove = Grid[label.Row - 1].GetRange(leftBound, rangeLength);
            adjacent.AddRange(adjacentAbove);
        }

        // get from same row
        {
            var adjacentSameRow = Grid[label.Row].GetRange(leftBound, rangeLength);
            adjacent.AddRange(adjacentSameRow);
        }
        
        // get from row below
        if (label.Row + 1 != Grid.Count)
        {
            var adjacentBelow = Grid[label.Row+1].GetRange(leftBound, rangeLength);
            adjacent.AddRange(adjacentBelow);
        }
        
        return adjacent;
    }
}
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
                var label = new Label
                {
                    Row = rowIndex,
                    Col = colStart,
                    Length = num.Length,
                    Value = int.Parse(num)
                };
                        
                labels.Add(label);
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
            if (label.AdjacentPoints.Any(point => !char.IsNumber(Grid[point.Row][point.Col]) && Grid[point.Row][point.Col] != '.'))
            {
                partNumbers.Add(label.Value);                
            }
        }

        return partNumbers;
    }

    public void CalculateLabelAdjacency()
    {
        foreach (var label in Labels)
        {
            label.AdjacentPoints = GetAdjacent(label);
        }
    }
    
    public List<int> GetGearRatios()
    {
        var gearRatios = new List<int>();

        for (var row = 0; row < Grid.Count; row++)
        {
            for (var col = 0; col < Grid[0].Count; col++)
            {
                if (Grid[row][col] == '*')
                {
                    var adjacentLabels = Labels.Where(l => l.IsAdjacentTo(new Point(row, col))).ToList();
                    if (adjacentLabels.Count() == 2)
                    {
                        gearRatios.Add(adjacentLabels[0].Value * adjacentLabels[1].Value);
                    }
                }
            }
        }

        return gearRatios;
    }
    
    List<Point> GetAdjacent(Label label)
    {
        var adjacent = new List<Point>();

        var leftBound = Math.Max(0, label.Col - 1);
        var rightBound = Math.Min(Width, label.Col + label.Length + 1);
        var rangeLength = rightBound - leftBound;
        
        // Console.WriteLine($"leftBound {leftBound}, rightBound {rightBound}");
        
        // get from row above
        if (label.Row != 0)
        {
            var adjacentAbove = 
                Enumerable.Range(leftBound, rangeLength)
                    .Select(col => new Point(label.Row - 1, col));

            adjacent.AddRange(adjacentAbove);
        }

        // get from same row
        {
            var adjacentAbove = 
                Enumerable.Range(leftBound, rangeLength)
                    .Select(col => new Point(label.Row, col));

            adjacent.AddRange(adjacentAbove);
        }
        
        // get from row below
        if (label.Row + 1 != Grid.Count)
        {
            var adjacentAbove = 
                Enumerable.Range(leftBound, rangeLength)
                    .Select(col => new Point(label.Row + 1, col));

            adjacent.AddRange(adjacentAbove);
        }
        
        return adjacent;
    }
}
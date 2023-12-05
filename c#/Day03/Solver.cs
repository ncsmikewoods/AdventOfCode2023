namespace Day03;

public class Solver
{
    private Schematic _schematic;

    public Solver()
    {
        GetInputs();
    }
    
    public int Solve1()
    {
        var partNumbers = _schematic.GetPartNumbers();
        // foreach (var partNumber in partNumbers)
        // {
        //     Console.WriteLine($"Part Number - {partNumber}");
        // }

        return partNumbers.Sum();
    }
    
    public double Solve2()
    {
        var gearRatios = _schematic.GetGearRatios();

        // foreach (var gearRatio in gearRatios)
        // {
        //     Console.WriteLine($"Gear Ratio: {gearRatio}");
        // }

        return gearRatios.Sum();
    }

    void GetInputs()
    {
        var lines = File.ReadAllLines("input.txt").ToList();
        var grid = lines.Select(l => l.ToList()).ToList();
        _schematic = new Schematic(grid);
        _schematic.CalculateLabelAdjacency();
    }
}
namespace Day05;

public class Solver
{
    private List<double> _seeds;
    private List<Mapper> _mappers;

    public Solver()
    {
        GetInputs();
    }

    public double Solve1()
    {
        var locations = new List<double>();
        
        foreach (var seed in _seeds)
        {
            var newValue = seed;
            // Console.Write($"Seed {seed} -   ");

            foreach (var mapper in _mappers)
            {
                newValue = mapper.Map(newValue);
            }
            locations.Add(newValue);
            // Console.WriteLine();
        }
        Console.WriteLine();

        foreach (var location in locations)
        {
            Console.WriteLine($"Location - {location}");
        }

        return locations.Min();
    }
    
    public int Solve2()
    {
        return 1;
    }

    void GetInputs()
    {
        var text = File.ReadAllText("input_short.txt");
        var sections = text.Split("\r\n\r\n");
        
        _seeds = sections[0].Split(' ').Skip(1).Select(double.Parse).ToList();
        _mappers = new List<Mapper>
        {
            new ("soil",sections[1]),
            new ("fertilizer", sections[2]),
            new ("water", sections[3]),
            new ("light", sections[4]),
            new ("temperature", sections[5]),
            new ("humidity", sections[6]),
            new ("location", sections[7])
        };
    }
}